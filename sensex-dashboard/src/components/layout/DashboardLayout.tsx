'use client';

export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <header className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 py-6">
          <h1 className="text-2xl font-bold text-gray-900">Ransomware Attack Analysis Report</h1>
          <p className="mt-1 text-sm text-gray-600">
            Comprehensive security analysis of ransomware attack patterns and high-risk alerts
          </p>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 py-6">
        {children}
      </main>

      {/* Footer */}
      <footer className="bg-white shadow mt-8">
        <div className="max-w-7xl mx-auto px-4 py-4">
          <p className="text-sm text-gray-600 text-center">
            Analysis based on Ransomware attacker test data (alerts_Ransomware_attacker_test_1014)
          </p>
        </div>
      </footer>
    </div>
  );
}
