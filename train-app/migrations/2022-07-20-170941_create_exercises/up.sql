CREATE TABLE IF NOT EXISTS exercises
(
	    id         UUID PRIMARY KEY        NOT NULL,
	    created_at TIMESTAMP DEFAULT now() NOT NULL,
	    message    TEXT                    NOT NULL
);
