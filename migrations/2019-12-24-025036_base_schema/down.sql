DROP TABLE IF EXISTS url_invitations;
DROP TABLE IF EXISTS email_invitations;
DROP TRIGGER IF EXISTS update_timestamp ON members;
DROP TABLE IF EXISTS members CASCADE;
DROP FUNCTION IF EXISTS trigger_update_timestamp;

-- TODO: will finish fleshing this out once I've fully spec'd the tables
