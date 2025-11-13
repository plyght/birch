CREATE TABLE policies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    priority INTEGER NOT NULL DEFAULT 0,
    enabled BOOLEAN NOT NULL DEFAULT true,
    
    scope TEXT NOT NULL CHECK (scope IN ('workspace', 'provider', 'secret')),
    provider_pattern TEXT,
    secret_pattern TEXT,
    
    rules JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(workspace_id, name)
);

CREATE INDEX idx_policies_workspace_id ON policies(workspace_id);
CREATE INDEX idx_policies_enabled ON policies(enabled);
CREATE INDEX idx_policies_priority ON policies(priority DESC);
CREATE INDEX idx_policies_scope ON policies(scope);

ALTER TABLE policies ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Workspace members can view policies"
    ON policies FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM workspace_members
            WHERE workspace_members.workspace_id = policies.workspace_id
            AND workspace_members.user_id = auth.uid()
        )
    );

CREATE POLICY "Owners and admins can manage policies"
    ON policies FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM workspace_members
            WHERE workspace_members.workspace_id = policies.workspace_id
            AND workspace_members.user_id = auth.uid()
            AND workspace_members.role IN ('owner', 'admin')
        )
    );

CREATE TRIGGER update_policies_updated_at BEFORE UPDATE ON policies
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

