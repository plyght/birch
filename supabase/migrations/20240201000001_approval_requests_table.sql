CREATE TABLE approval_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    requester_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    
    action_type TEXT NOT NULL CHECK (action_type IN ('rotation', 'rollback', 'policy_change')),
    provider TEXT NOT NULL,
    secret_name TEXT,
    
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'approved', 'rejected', 'expired', 'cancelled')),
    
    approved_by UUID REFERENCES auth.users(id),
    approved_at TIMESTAMPTZ,
    rejected_by UUID REFERENCES auth.users(id),
    rejected_at TIMESTAMPTZ,
    rejection_reason TEXT,
    
    expires_at TIMESTAMPTZ NOT NULL,
    
    metadata JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_approval_requests_workspace_id ON approval_requests(workspace_id);
CREATE INDEX idx_approval_requests_status ON approval_requests(status);
CREATE INDEX idx_approval_requests_requester_id ON approval_requests(requester_id);
CREATE INDEX idx_approval_requests_expires_at ON approval_requests(expires_at);
CREATE INDEX idx_approval_requests_created_at ON approval_requests(created_at);

ALTER TABLE approval_requests ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Workspace members can view approval requests"
    ON approval_requests FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM workspace_members
            WHERE workspace_members.workspace_id = approval_requests.workspace_id
            AND workspace_members.user_id = auth.uid()
        )
    );

CREATE POLICY "Users with rotate permission can create approval requests"
    ON approval_requests FOR INSERT
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM workspace_members
            WHERE workspace_members.workspace_id = approval_requests.workspace_id
            AND workspace_members.user_id = auth.uid()
            AND workspace_members.role IN ('owner', 'admin', 'operator')
        )
    );

CREATE POLICY "Users with approve permission can update approval requests"
    ON approval_requests FOR UPDATE
    USING (
        EXISTS (
            SELECT 1 FROM workspace_members
            WHERE workspace_members.workspace_id = approval_requests.workspace_id
            AND workspace_members.user_id = auth.uid()
            AND workspace_members.role IN ('owner', 'admin')
        )
    );

CREATE TRIGGER update_approval_requests_updated_at BEFORE UPDATE ON approval_requests
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE OR REPLACE FUNCTION expire_old_approval_requests()
RETURNS void AS $$
BEGIN
    UPDATE approval_requests
    SET status = 'expired', updated_at = NOW()
    WHERE status = 'pending'
    AND expires_at < NOW();
END;
$$ LANGUAGE plpgsql;

