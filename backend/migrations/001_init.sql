-- Create snippets table
CREATE TABLE IF NOT EXISTS snippets (
    id VARCHAR(255) PRIMARY KEY,
    content TEXT NOT NULL,
    visibility VARCHAR(20) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE,
    password_hash VARCHAR(255),
    delete_token VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    size INTEGER NOT NULL,
    CONSTRAINT check_visibility CHECK (visibility IN ('public', 'unlisted', 'private'))
);

-- Create indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_expires_at ON snippets(expires_at) WHERE expires_at IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_created_at ON snippets(created_at);

-- Add comment for documentation
COMMENT ON TABLE snippets IS 'Stores text snippets with metadata for access control and expiration';
COMMENT ON COLUMN snippets.id IS 'Unique non-sequential identifier (UUID v4)';
COMMENT ON COLUMN snippets.content IS 'UTF-8 text content of the snippet';
COMMENT ON COLUMN snippets.visibility IS 'Access control level: public, unlisted, or private';
COMMENT ON COLUMN snippets.expires_at IS 'Expiration timestamp, NULL for never-expiring snippets';
COMMENT ON COLUMN snippets.password_hash IS 'Argon2 hash of password for private snippets';
COMMENT ON COLUMN snippets.delete_token IS 'Token required for manual deletion';
COMMENT ON COLUMN snippets.created_at IS 'Creation timestamp';
COMMENT ON COLUMN snippets.size IS 'Content size in bytes';
