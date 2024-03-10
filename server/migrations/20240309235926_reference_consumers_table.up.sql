ALTER TABLE subscriptions ADD CONSTRAINT consumers_fk FOREIGN KEY (consumerId) REFERENCES consumers(id);
