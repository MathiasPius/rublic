-- Your SQL goes here

CREATE TABLE rublic.domains (
	id CHAR(36) NOT NULL,
	fqdn varchar(256) NOT NULL,
	CONSTRAINT domains_PK PRIMARY KEY (id)
);

CREATE TABLE rublic.domain_groups (
	id CHAR(36) NOT NULL,
	friendly_name VARCHAR(256) NOT NULL,
	CONSTRAINT domain_groups_PK PRIMARY KEY (id)
);

CREATE TABLE rublic.domain_group_mappings (
    domain_id CHAR(36) NOT NULL,
    group_id CHAR(36) NOT NULL,
    CONSTRAINT domain_group_mappings_PL PRIMARY KEY (domain_id, group_id),
	CONSTRAINT domain_FK FOREIGN KEY (domain_id) REFERENCES rublic.domains(id),
	CONSTRAINT domain_groups_FK FOREIGN KEY (group_id) REFERENCES rublic.domain_groups(id)
);