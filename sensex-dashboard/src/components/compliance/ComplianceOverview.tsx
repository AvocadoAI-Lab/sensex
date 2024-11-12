'use client';
import { Doughnut } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  ArcElement,
  Tooltip,
  Legend,
  ChartOptions,
} from 'chart.js';
import type { Root } from '@/types/alerts';

ChartJS.register(ArcElement, Tooltip, Legend);

interface ComplianceOverviewProps {
  data: Root;
}

export default function ComplianceOverview({ data }: ComplianceOverviewProps) {
  // Filter for high severity alerts with SCA data
  const highSeverityScaData = data.hits.hits.filter(
    hit => hit._source.rule.level >= 10 && hit._source.data?.sca
  );

  // Count SCA checks by result for high severity alerts
  const scaResults = highSeverityScaData.reduce(
    (acc: { passed: number; failed: number; invalid: number }, hit) => {
      const sca = hit._source.data?.sca;
      if (sca) {
        if (sca.passed) acc.passed += parseInt(sca.passed) || 0;
        if (sca.failed) acc.failed += parseInt(sca.failed) || 0;
        if (sca.invalid) acc.invalid += parseInt(sca.invalid) || 0;
      }
      return acc;
    },
    { passed: 0, failed: 0, invalid: 0 }
  );

  // Get unique policies and their frequencies
  const policyFrequency = highSeverityScaData.reduce((acc: { [key: string]: number }, hit) => {
    const policy = hit._source.data?.sca?.policy;
    if (policy) {
      acc[policy] = (acc[policy] || 0) + 1;
    }
    return acc;
  }, {});

  const chartData = {
    labels: ['Passed', 'Failed', 'Invalid'],
    datasets: [
      {
        data: [scaResults.passed, scaResults.failed, scaResults.invalid],
        backgroundColor: [
          'rgba(34, 197, 94, 0.8)',  // green
          'rgba(239, 68, 68, 0.8)',  // red
          'rgba(234, 179, 8, 0.8)',  // yellow
        ],
        borderColor: [
          'rgb(34, 197, 94)',
          'rgb(239, 68, 68)',
          'rgb(234, 179, 8)',
        ],
        borderWidth: 1,
      },
    ],
  };

  const chartOptions: ChartOptions<'doughnut'> = {
    responsive: true,
    plugins: {
      legend: {
        position: 'top',
      },
      title: {
        display: true,
        text: 'High Severity Compliance Check Results',
        font: {
          size: 16,
        },
      },
    },
    cutout: '60%',
  };

  // Calculate compliance metrics
  const totalChecks = scaResults.passed + scaResults.failed + scaResults.invalid;
  const complianceRate = totalChecks > 0 
    ? Math.round((scaResults.passed / totalChecks) * 100) 
    : 0;
  const failureRate = totalChecks > 0
    ? Math.round((scaResults.failed / totalChecks) * 100)
    : 0;

  return (
    <div className="space-y-6">
      {/* Compliance Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="bg-white p-6 rounded-lg shadow">
          <h3 className="text-sm font-medium text-gray-500">Total Compliance Checks</h3>
          <p className="mt-2 text-3xl font-semibold text-gray-900">{totalChecks}</p>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <h3 className="text-sm font-medium text-gray-500">Compliance Rate</h3>
          <p className={`mt-2 text-3xl font-semibold ${complianceRate >= 80 ? 'text-green-600' : 'text-red-600'}`}>
            {complianceRate}%
          </p>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <h3 className="text-sm font-medium text-gray-500">Critical Failure Rate</h3>
          <p className={`mt-2 text-3xl font-semibold ${failureRate <= 20 ? 'text-green-600' : 'text-red-600'}`}>
            {failureRate}%
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Compliance Chart */}
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="h-[300px] flex items-center justify-center">
            <Doughnut data={chartData} options={chartOptions} />
          </div>
        </div>

        {/* Policy Coverage */}
        <div className="bg-white p-6 rounded-lg shadow">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Policy Coverage</h3>
          <div className="space-y-4">
            {Object.entries(policyFrequency)
              .sort(([, a], [, b]) => b - a)
              .map(([policy, count]) => (
                <div key={policy} className="flex items-center">
                  <div className="flex-1">
                    <div className="flex justify-between mb-1">
                      <span className="text-sm font-medium text-gray-700">{policy}</span>
                      <span className="text-sm text-gray-500">{count} checks</span>
                    </div>
                    <div className="w-full bg-gray-200 rounded-full h-2">
                      <div
                        className="bg-blue-600 h-2 rounded-full"
                        style={{
                          width: `${Math.round((count / totalChecks) * 100)}%`,
                        }}
                      />
                    </div>
                  </div>
                </div>
              ))}
          </div>
        </div>
      </div>

      {/* Compliance Notes */}
      <div className="bg-white p-6 rounded-lg shadow">
        <h3 className="text-lg font-medium text-gray-900 mb-4">Compliance Insights</h3>
        <div className="prose max-w-none">
          <ul className="space-y-2 text-gray-600">
            <li>
              {complianceRate >= 80 
                ? "Strong compliance performance with over 80% pass rate"
                : "Attention needed: Compliance rate below recommended threshold"}
            </li>
            <li>
              {failureRate <= 20
                ? "Acceptable failure rate within normal range"
                : "Critical: High failure rate requires immediate attention"}
            </li>
            <li>
              Total of {Object.keys(policyFrequency).length} unique compliance policies monitored
            </li>
          </ul>
        </div>
      </div>
    </div>
  );
}
