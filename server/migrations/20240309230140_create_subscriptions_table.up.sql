CREATE TABLE subscriptions (
    consumerId INTEGER NOT NULL,
    webhookName VARCHAR(200),
    url TEXT NOT NULL,
    secret VARCHAR(75) NOT NULL,
    subscribedTo INTEGER[],

    PRIMARY KEY (consumerId)
);
