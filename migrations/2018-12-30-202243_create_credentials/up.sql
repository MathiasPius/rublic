-- Your SQL goes here

CREATE TABLE rublic.access_credentials (
	id CHAR(36) NOT NULL,
	friendly_name VARCHAR(256) NOT NULL,
    hashed_key CHAR(64) NOT NULL,
	CONSTRAINT credentials_PK PRIMARY KEY (id)
);

CREATE TABLE rublic.access_groups (
	id CHAR(36) NOT NULL,
	friendly_name VARCHAR(256) NOT NULL,
	CONSTRAINT access_groups_PK PRIMARY KEY (id)
);

CREATE TABLE rublic.credential_group_mappings (
	id CHAR(36) NOT NULL,
    access_credential_id CHAR(36) NOT NULL,
    access_group_id CHAR(36) NOT NULL,
    -- CONSTRAINT credential_group_mappings_PL PRIMARY KEY (access_credential_id, access_group_id),
	CONSTRAINT credential_group_mappings_PK PRIMARY KEY (id),
	CONSTRAINT access_credentials_FK FOREIGN KEY (access_credential_id) REFERENCES rublic.access_credentials(id),
	CONSTRAINT access_groups_FK FOREIGN KEY (access_group_id) REFERENCES rublic.access_groups(id)
);