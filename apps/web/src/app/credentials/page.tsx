'use client'

import { motion } from 'framer-motion'
import DashboardLayout from '@/components/layout/DashboardLayout'
import Card from '@/components/ui/Card'
import Button from '@/components/ui/Button'
import { IconKey, IconPlus } from '@tabler/icons-react'

export default function CredentialsPage() {
  return (
    <DashboardLayout>
      <div className="space-y-6">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="flex items-center justify-between"
        >
          <div>
            <h1 className="text-4xl font-serif font-semibold bg-gradient-to-r from-accent-blue to-accent-purple bg-clip-text text-transparent mb-2">
              Credentials
            </h1>
            <p className="text-text-secondary text-lg">
              Manage provider credentials and configurations
            </p>
          </div>
          <Button variant="gradient">
            <IconPlus className="w-5 h-5" />
            Add Credential
          </Button>
        </motion.div>

        <Card>
          <div className="text-center py-16">
            <motion.div
              initial={{ scale: 0.9, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              transition={{ delay: 0.2 }}
            >
              <div className="w-20 h-20 bg-gradient-to-br from-green-500 to-green-600 rounded-2xl mx-auto mb-6 flex items-center justify-center">
                <IconKey className="w-10 h-10 text-white" />
              </div>
              <h3 className="text-2xl font-serif font-semibold text-text-primary mb-2">
                No credentials yet
              </h3>
              <p className="text-text-secondary mb-6 max-w-md mx-auto">
                Add credentials to start managing your provider configurations and secret rotations.
              </p>
              <Button variant="gradient" size="lg">
                <IconPlus className="w-5 h-5" />
                Add Your First Credential
              </Button>
            </motion.div>
          </div>
        </Card>
      </div>
    </DashboardLayout>
  )
}
