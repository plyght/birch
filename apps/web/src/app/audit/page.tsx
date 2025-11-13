'use client'

import DashboardLayout from '@/components/layout/DashboardLayout'
import Card, { CardHeader } from '@/components/ui/Card'
import Table, { TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/Table'
import Badge from '@/components/ui/Badge'
import Button from '@/components/ui/Button'

// Sample data for demonstration
const sampleLogs = [
  {
    id: '1',
    action: 'workspace.created',
    user: 'user@example.com',
    timestamp: new Date().toISOString(),
    status: 'success',
  },
  {
    id: '2',
    action: 'credential.added',
    user: 'user@example.com',
    timestamp: new Date(Date.now() - 3600000).toISOString(),
    status: 'success',
  },
]

export default function AuditPage() {
  return (
    <DashboardLayout>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-4xl font-serif font-semibold text-text-primary mb-2">
              Audit Logs
            </h1>
            <p className="text-text-secondary">
              Track all changes and activities across your workspaces
            </p>
          </div>
          <Button variant="secondary">Export Logs</Button>
        </div>

        <Card>
          <CardHeader title="Activity Log" description="All system activities" />
          <Table>
            <TableHeader>
              <tr>
                <TableHead>Action</TableHead>
                <TableHead>User</TableHead>
                <TableHead>Timestamp</TableHead>
                <TableHead>Status</TableHead>
              </tr>
            </TableHeader>
            <TableBody>
              {sampleLogs.map((log) => (
                <TableRow key={log.id}>
                  <TableCell>
                    <code className="text-sm">{log.action}</code>
                  </TableCell>
                  <TableCell>{log.user}</TableCell>
                  <TableCell>
                    {new Date(log.timestamp).toLocaleString()}
                  </TableCell>
                  <TableCell>
                    <Badge variant="success">{log.status}</Badge>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </Card>
      </div>
    </DashboardLayout>
  )
}

