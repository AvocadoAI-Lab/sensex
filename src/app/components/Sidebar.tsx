'use client';
import Link from 'next/link';
import { usePathname } from 'next/navigation';

export default function Sidebar() {
  const pathname = usePathname();

  const isActive = (path: string) => {
    return pathname === path ? 'bg-blue-100 text-blue-700' : 'text-gray-600 hover:bg-gray-100';
  };

  return (
    <div className="w-64 bg-white h-screen fixed left-0 top-0 border-r border-gray-200">
      <div className="p-4">
        <h1 className="text-xl font-bold text-gray-800">Wazuh Dashboard</h1>
      </div>
      <nav className="mt-4">
        <Link 
          href="/"
          className={`block px-4 py-2 text-sm ${isActive('/')}`}
        >
          Home
        </Link>
        <Link 
          href="/dev"
          className={`block px-4 py-2 text-sm ${isActive('/dev')}`}
        >
          Dev
        </Link>
      </nav>
    </div>
  );
}
