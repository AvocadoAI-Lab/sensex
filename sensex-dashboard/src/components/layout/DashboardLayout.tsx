'use client';
import DownloadButton from '../common/DownloadButton';

export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100">
      {/* Header */}
      <header className="bg-white shadow-md border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-6 py-8">
          <div className="flex justify-between items-center">
            <div>
              <h1 className="text-3xl font-bold text-gray-900 tracking-tight">
                Group Security Analysis Dashboard
              </h1>
              <p className="mt-2 text-base text-gray-600">
                Comprehensive security analysis and monitoring across all agents
              </p>
            </div>
            <div className="ml-4">
              <DownloadButton />
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-6 py-8">
        <div className="bg-white rounded-xl shadow-lg p-8">
          {children}
        </div>
      </main>

      {/* Footer */}
      <footer className="bg-white border-t border-gray-200 mt-8">
        <div className="max-w-7xl mx-auto px-6 py-6">
          <p className="text-sm text-gray-600 text-center">
            Real-time security analysis dashboard for monitoring and analyzing group-wide security events
          </p>
        </div>
      </footer>
    </div>
  );
}
