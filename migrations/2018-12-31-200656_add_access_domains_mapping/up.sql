-- Your SQL goes here

CREATE TABLE IF NOT EXISTS rublic.group_permissions (
	id CHAR(36) NOT NULL,
	permission VARCHAR(64) NOT NULL,
    access_group_id CHAR(36) NOT NULL,
    domain_group_id CHAR(36) NOT NULL,
	CONSTRAINT access_domain_mappings_PK PRIMARY KEY (id),
	CONSTRAINT group_permissions_domain_FK FOREIGN KEY (domain_group_id) REFERENCES rublic.domain_groups(id),
	CONSTRAINT group_permissions_access_FK FOREIGN KEY (access_group_id) REFERENCES rublic.access_groups(id)
);