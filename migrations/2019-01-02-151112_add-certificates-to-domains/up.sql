-- Your SQL goes here

CREATE TABLE IF NOT EXISTS rublic.certificates (
    id INT NOT NULL,
    domain_id CHAR(36) NOT NULL,
    not_before DATETIME NOT NULL,
    not_after DATETIME NOT NULL,
    CONSTRAINT certificates_PK PRIMARY KEY (domain_id, id),
    CONSTRAINT certificates_domain_FK FOREIGN KEY (domain_id) REFERENCES rublic.domains(id) ON DELETE CASCADE
)