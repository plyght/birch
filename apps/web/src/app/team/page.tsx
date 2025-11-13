'use client'

import DashboardLayout from '@/components/layout/DashboardLayout'
import Card, { CardHeader } from '@/components/ui/Card'
import Button from '@/components/ui/Button'
import Table, { TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/Table'
import Badge from '@/components/ui/Badge'
import { useEffect, useState } from 'react'
import { supabase } from '@/lib/supabase'

export default function TeamPage() {
  const [user, setUser] = useState<any>(null)

  useEffect(() => {
    const loadUser = async () => {
      const { data: { session } } = await supabase.auth.getSession()
      setUser(session?.user)
    }
    loadUser()
  }, [])

  return (
    <DashboardLayout>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-4xl font-serif font-semibold text-text-primary mb-2">
              Team
            </h1>
            <p className="text-text-secondary">
              Manage team members and permissions
            </p>
          </div>
          <Button>Invite Member</Button>
        </div>

        <Card>
          <CardHeader title="Team Members" description="Manage access and roles" />
          <Table>
            <TableHeader>
              <tr>
                <TableHead>Email</TableHead>
                <TableHead>Role</TableHead>
                <TableHead>Joined</TableHead>
                <TableHead>Actions</TableHead>
              </tr>
            </TableHeader>
            <TableBody>
              <TableRow>
                <TableCell>{user?.email || 'Loading...'}</TableCell>
                <TableCell>
                  <Badge variant="success">Owner</Badge>
                </TableCell>
                <TableCell>{new Date().toLocaleDateString()}</TableCell>
                <TableCell>
                  <span className="text-text-tertiary text-sm">-</span>
                </TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </Card>
      </div>
    </DashboardLayout>
  )
}

