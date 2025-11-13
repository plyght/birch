'use client'

import DashboardLayout from '@/components/layout/DashboardLayout'
import Card from '@/components/ui/Card'
import Button from '@/components/ui/Button'

export default function PoliciesPage() {
  return (
    <DashboardLayout>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-4xl font-serif font-semibold text-text-primary mb-2">
              Policies
            </h1>
            <p className="text-text-secondary">
              Define and manage rotation policies
            </p>
          </div>
          <Button>Create Policy</Button>
        </div>

        <Card>
          <div className="text-center py-16">
            <div className="w-16 h-16 bg-purple-100 rounded-full mx-auto mb-4 flex items-center justify-center">
              <svg
                className="w-8 h-8 text-purple-600"
                fill="none"
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
              </svg>
            </div>
            <h3 className="text-2xl font-serif font-semibold text-text-primary mb-2">
              No policies configured
            </h3>
            <p className="text-text-secondary mb-6 max-w-md mx-auto">
              Create rotation policies to automate secret management and compliance.
            </p>
            <Button>Create Your First Policy</Button>
          </div>
        </Card>
      </div>
    </DashboardLayout>
  )
}

