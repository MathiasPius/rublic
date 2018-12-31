-- Your SQL goes here

CREATE TABLE IF NOT EXISTS rublic.domain_entries (
	id CHAR(36) NOT NULL,
	fqdn varchar(256) NOT NULL,
	CONSTRAINT domain_entries_PK PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS rublic.domain_groups (
	id CHAR(36) NOT NULL,
	friendly_name VARCHAR(256) NOT NULL,
	CONSTRAINT domain_groups_PK PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS rublic.entry_group_mappings (
	id CHAR(36) NOT NULL,
    domain_entry_id CHAR(36) NOT NULL,
    domain_group_id CHAR(36) NOT NULL,
    CONSTRAINT entry_group_mappings_PK PRIMARY KEY (id),
	CONSTRAINT domain_entries_FK FOREIGN KEY (domain_entry_id) REFERENCES rublic.domain_entries(id),
	CONSTRAINT domain_groups_FK FOREIGN KEY (domain_group_id) REFERENCES rublic.domain_groups(id)
);