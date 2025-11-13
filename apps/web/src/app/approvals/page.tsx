'use client'

import DashboardLayout from '@/components/layout/DashboardLayout'
import Card from '@/components/ui/Card'

export default function ApprovalsPage() {
  return (
    <DashboardLayout>
      <div className="space-y-6">
        <div>
          <h1 className="text-4xl font-serif font-semibold text-text-primary mb-2">
            Approvals
          </h1>
          <p className="text-text-secondary">
            Review and manage approval requests
          </p>
        </div>

        <Card>
          <div className="text-center py-16">
            <div className="w-16 h-16 bg-blue-100 rounded-full mx-auto mb-4 flex items-center justify-center">
              <svg
                className="w-8 h-8 text-blue-600"
                fill="none"
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z" />
              </svg>
            </div>
            <h3 className="text-2xl font-serif font-semibold text-text-primary mb-2">
              No pending approvals
            </h3>
            <p className="text-text-secondary max-w-md mx-auto">
              Approval requests will appear here when team members request changes that need authorization.
            </p>
          </div>
        </Card>
      </div>
    </DashboardLayout>
  )
}

