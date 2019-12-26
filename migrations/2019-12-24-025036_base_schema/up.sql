CREATE EXTENSION pgcrypto;

-- Create a useful function for automatically updating the updated_at column on each table we apply
-- this to.
CREATE OR REPLACE FUNCTION trigger_update_timestamp()
  RETURNS TRIGGER AS $tut$
  BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
  END;
$tut$ LANGUAGE plpgsql;

-- This the primary membership table containing all the most recent contact details for each camp
-- member. This specifically does not associate with sign-ups as this can be re-used year to year
-- simplifying the process. This is also being intentionally separated from authentication material
-- as we may not have brought them through a sign-up process with this system but still have
-- information about them that we want to track (like years burned with MindShark).
DROP TABLE IF EXISTS members;
CREATE TABLE members (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),

  real_name VARCHAR(256) NOT NULL,
  playa_name VARCHAR(96),

  email VARCHAR(256) NOT NULL,
  phone VARCHAR(32) NOT NULL,

  location VARCHAR(256),

  years_burned INT NOT NULL DEFAULT 0,

  -- May want to pull these out into a separate tables as arrays of blobs can suck... but there
  -- won't be a lot of churn so this should be fine.
  known_allergies TEXT ARRAY,
  known_medications TEXT ARRAY,
  dietary_restrictions TEXT ARRAY,

  invited_by UUID REFERENCES members(id),

  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Emails should be unique
CREATE UNIQUE INDEX members_email ON members(email);

DROP TABLE IF EXISTS emergency_contacts;
CREATE TABLE emergency_contacts (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),

  member_id UUID NOT NULL REFERENCES members(id),

  name VARCHAR(256) NOT NULL,
  phone_number VARCHAR(32),

  -- If we have multiple emergency contacts one of them should be designated first contact
  first_contact BOOLEAN NOT NULL DEFAULT FALSE
);

-- Associate the auto-updating timestamp function with the membership table
DROP TRIGGER IF EXISTS update_timestamp ON members;
CREATE TRIGGER update_timestamp
  BEFORE UPDATE ON members
  FOR EACH ROW
    EXECUTE PROCEDURE trigger_update_timestamp();

-- We normally ask people what years they've burned with MindShark but that doesn't actually matter
-- if we already have that information. I can pre-populate this with our existing survey responses
-- and general shark knowledge so we don't have to ask that ever again.
DROP TABLE IF EXISTS year_configuration;
CREATE TABLE year_configuration (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),

  year INT NOT NULL,
  maximum_attendees INT NOT NULL,
  dues_amount INT NOT NULL,

  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

DROP TRIGGER IF EXISTS update_timestamp ON year_configuration;
CREATE TRIGGER update_timestamp
  BEFORE UPDATE ON year_configuration
  FOR EACH ROW
    EXECUTE PROCEDURE trigger_update_timestamp();

CREATE UNIQUE INDEX year_configuration_year ON year_configuration(year);

DROP TABLE IF EXISTS signups;
CREATE TABLE signups (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),

  member_id UUID NOT NULL REFERENCES members(id),
  year_id UUID NOT NULL REFERENCES year_configuration(id),

  attendance_probability INT NOT NULL,

  -- These times should always be Reno local so no timezones, they'll only complicate things.
  -- Considered using a DATE type here and that's probably all we'll ask about in the primary
  -- sign-up form but if we can get flight arrival times and the like we might be able to schedule
  -- rides between people with a solver...
  expected_arrival TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  expected_departure TIMESTAMP WITHOUT TIME ZONE NOT NULL,

  sleeping_arrangement VARCHAR(64) NOT NULL,

  -- These are ternary values and we can use 'NULL' as Maybe?
  willing_to_early_arrival BOOLEAN DEFAULT FALSE,
  willing_to_post_burn BOOLEAN DEFAULT FALSE,

  -- Various checkboxes that should be confirmed every year...
  read_essential_mindshark BOOLEAN NOT NULL DEFAULT FALSE,
  read_project_descriptions BOOLEAN NOT NULL DEFAULT FALSE,

  will_pay_dues BOOLEAN NOT NULL DEFAULT FALSE,
  will_perform_duties BOOLEAN NOT NULL DEFAULT FALSE,
  will_tear_down BOOLEAN NOT NULL DEFAULT FALSE,

  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

  -- Note: going to roll shift pod into personal yurt as it doesn't matter placement wise but the
  -- specificity doesn't need to extend the type definition name
  CONSTRAINT valid_sleeping_arrangement CHECK (
    sleeping_arrangement IN (
      'boner_estates', 'communal_yurt', 'jan_rv', 'personal_yurt', 'tent', 'other'
    )
  )
);

DROP TRIGGER IF EXISTS update_timestamp ON signups;
CREATE TRIGGER update_timestamp
  BEFORE UPDATE ON signups
  FOR EACH ROW
    EXECUTE PROCEDURE trigger_update_timestamp();

DROP TABLE IF EXISTS ticket_information;
CREATE TABLE ticket_information (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),

  member_id UUID NOT NULL REFERENCES members(id),
  year_id UUID NOT NULL REFERENCES year_configuration(id),
  ticket_source_id UUID REFERENCES ticket_information(id),

  ticket_status VARCHAR(64) NOT NULL DEFAULT 'unticketed',
  extra_tickets INT NOT NULL DEFAULT 0,

  vehicle_pass BOOLEAN NOT NULL DEFAULT FALSE,
  extra_vehicle_passes INT NOT NULL DEFAULT 0,

  pool_participant BOOLEAN NOT NULL DEFAULT FALSE,

  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

  CONSTRAINT valid_ticket_status CHECK (
    ticket_status IN (
      'camp_grant', 'pool_purchaser', 'pool_recipient', 'self_ticketed', 'unticketed'
    )
  )
);

DROP TRIGGER IF EXISTS update_timestamp ON ticket_information;
CREATE TRIGGER update_timestamp
  BEFORE UPDATE ON ticket_information
  FOR EACH ROW
    EXECUTE PROCEDURE trigger_update_timestamp();

-- Email invitations can be handled automatically by the app, handles direct known sign-ups and
-- corruptions
DROP TABLE IF EXISTS email_invitations;
CREATE TABLE email_invitations (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),

  email VARCHAR(256) NOT NULL,
  -- We can have members that don't officially have an invited association such as general randoms
  -- or people that were present before we tracked this.
  invited_by UUID REFERENCES members(id),
  token VARCHAR(64) NOT NULL,

  expires_at TIMESTAMP WITH TIME ZONE,

  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  last_sent_at TIMESTAMP WITH TIME ZONE
);

-- URLs can be re-used or be one-offs to handle both general sign-ups and direct invitations pass
-- through means other than email.
DROP TABLE IF EXISTS url_invitations;
CREATE TABLE url_invitations (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),

  invited_by UUID REFERENCES members(id),
  token VARCHAR(64) NOT NULL,

  remaining_uses INTEGER NOT NULL DEFAULT 1,
  expires_at TIMESTAMP WITH TIME ZONE,

  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  last_sent_at TIMESTAMP WITH TIME ZONE
);


-- TODO: Dues/financials and project memberships


DROP VIEW IF EXISTS current_members_view;
CREATE VIEW current_members_view AS
  SELECT
      -- This is the information that seems pretty reasonable to allow everyone to see for the time
      -- being, can always be adjusted later.
      members.id AS id, members.real_name AS real_name, members.playa_name AS playa_name,
      members.location AS location, signups.expected_arrival AS expected_arrival,
      signups.expected_departure AS expected_departure

    FROM signups
      INNER JOIN members ON members.id = signups.member_id
      INNER JOIN year_configuration ON signups.year_id = year_configuration.id
    WHERE
      -- Only show the most recent year to current members
      year_configuration.id = (SELECT id FROM year_configuration ORDER BY year DESC LIMIT 1);

-- TODO: Remove this it's a reference test data bit and should be set properly before the app goes live
INSERT INTO year_configuration (year, maximum_attendees, dues_amount)
  VALUES
    (2019, 75, 375),
    (2020, 75, 375)
  ;
