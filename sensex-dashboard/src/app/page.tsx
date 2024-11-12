'use client';
import { useEffect, useState } from 'react';
import DashboardLayout from '@/components/layout/DashboardLayout';
import AlertSummary from '@/components/alerts/AlertSummary';
import RuleAnalytics from '@/components/rules/RuleAnalytics';
import type { Root } from '@/types/alerts';

export default function Home() {
  const [data, setData] = useState<Root | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetch('/data/alerts.json')
      .then(response => {
        if (!response.ok) {
          throw new Error('Failed to fetch data');
        }
        return response.json();
      })
      .then((jsonData: Root) => {
        setData(jsonData);
        setLoading(false);
      })
      .catch(err => {
        console.error('Error loading data:', err);
        setError(err.message);
        setLoading(false);
      });
  }, []);

  if (loading) {
    return (
      <DashboardLayout>
        <div className="flex items-center justify-center min-h-screen">
          <div className="text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-red-500 mx-auto"></div>
            <p className="mt-4 text-gray-600">Loading ransomware attack data...</p>
          </div>
        </div>
      </DashboardLayout>
    );
  }

  if (error || !data) {
    return (
      <DashboardLayout>
        <div className="flex items-center justify-center min-h-screen">
          <div className="text-center text-red-600">
            <h2 className="text-xl font-semibold mb-2">Error Loading Data</h2>
            <p>{error || 'Failed to load attack data'}</p>
          </div>
        </div>
      </DashboardLayout>
    );
  }

  return (
    <DashboardLayout>
      <div className="space-y-12">
        {/* Attack Overview Section */}
        <section className="bg-white rounded-lg shadow-lg p-6">
          <div className="border-b pb-4 mb-6">
            <h2 className="text-xl font-semibold text-gray-900">Attack Overview</h2>
            <p className="text-sm text-gray-500 mt-1">
              High severity alerts and attack patterns detected during the ransomware test
            </p>
          </div>
          <AlertSummary data={data} />
        </section>

        {/* Attack Pattern Analysis */}
        <section className="bg-white rounded-lg shadow-lg p-6">
          <div className="border-b pb-4 mb-6">
            <h2 className="text-xl font-semibold text-gray-900">Attack Pattern Analysis</h2>
            <p className="text-sm text-gray-500 mt-1">
              Distribution and trends of attack techniques and malicious activities
            </p>
          </div>
          <RuleAnalytics data={data} />
        </section>

        {/* Report Summary */}
        <section className="bg-white rounded-lg shadow-lg p-6">
          <h2 className="text-xl font-semibold text-gray-900 mb-4">Attack Analysis Summary</h2>
          <div className="prose max-w-none">
            <p className="text-gray-600">
              This report analyzes high-severity security alerts triggered during a simulated ransomware attack test.
              The data provides insights into attack patterns, detection effectiveness, and potential security implications.
            </p>
            <ul className="mt-4 space-y-2 text-gray-600">
              <li>Total alerts analyzed: {data.hits.total.value}</li>
              <li>Analysis period: {new Date(data.hits.hits[data.hits.hits.length - 1]._source.timestamp).toLocaleDateString()} - {new Date(data.hits.hits[0]._source.timestamp).toLocaleDateString()}</li>
              <li>Processing time: {data.took}ms</li>
            </ul>
          </div>
        </section>
      </div>
    </DashboardLayout>
  );
}
