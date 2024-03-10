CREATE TABLE subscriptions (
    uuid UUID NOT NULL,
    consumerId UUID NOT NULL,
    webhookName VARCHAR(200),
    url TEXT NOT NULL,
    secret VARCHAR(75) NOT NULL,
    subscribedTo INTEGER[],

    PRIMARY KEY (uuid)
);
