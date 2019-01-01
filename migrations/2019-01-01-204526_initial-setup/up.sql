-- Your SQL goes here


CREATE TABLE IF NOT EXISTS rublic.domains (
    id CHAR(36) NOT NULL,
	fqdn VARCHAR(256) NOT NULL,
	CONSTRAINT domains_PK PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS rublic.users (
    id CHAR(36) NOT NULL,
	friendly_name VARCHAR(256) NOT NULL,
	hashed_key CHAR(64) NOT NULL,
    CONSTRAINT users_PK PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS rublic.groups (
	id CHAR(36) NOT NULL,
	friendly_name VARCHAR(256) NOT NULL,
    permission VARCHAR(256) NOT NULL,
	CONSTRAINT groups_PK PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS rublic.domain_group_mappings (
	domain_id CHAR(36) NOT NULL,
    group_id CHAR(36) NOT NULL,
    CONSTRAINT domain_group_mappings_PK PRIMARY KEY (domain_id, group_id),
	CONSTRAINT mapping_domain_FK FOREIGN KEY (domain_id) REFERENCES rublic.domains(id),
	CONSTRAINT mapping_domain_group_FK FOREIGN KEY (group_id) REFERENCES rublic.groups(id)
);

CREATE TABLE IF NOT EXISTS rublic.user_group_mappings (
	user_id CHAR(36) NOT NULL,
    group_id CHAR(36) NOT NULL,
    CONSTRAINT domain_group_mappings_PK PRIMARY KEY (user_id, group_id),
	CONSTRAINT mapping_users_FK FOREIGN KEY (user_id) REFERENCES rublic.users(id),
	CONSTRAINT mapping_users_group_FK FOREIGN KEY (group_id) REFERENCES rublic.groups(id)
);