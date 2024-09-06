CREATE TABLE IF NOT EXISTS "roulette"
(
  uuid          uuid DEFAULT gen_random_uuid(),
  admin_code    uuid DEFAULT gen_random_uuid(),
  name          VARCHAR(100),
  email         VARCHAR(100),
  phone         VARCHAR(20),
  description   VARCHAR,
  PRIMARY KEY (uuid)
);

CREATE TABLE IF NOT EXISTS "user"
(
  uuid          uuid DEFAULT gen_random_uuid(),
  first_name    VARCHAR(100),
  last_name     VARCHAR(100),
  email         VARCHAR(100),
  phone         VARCHAR(20),
  PRIMARY KEY (uuid)
);

CREATE TABLE IF NOT EXISTS "roulette_user"
(
  user_uuid         uuid REFERENCES "user" (uuid) ON UPDATE CASCADE ON DELETE CASCADE,
  roulette_uuid     uuid REFERENCES "roulette" (uuid) ON UPDATE CASCADE,
  frequency         int NOT NULL DEFAULT 2,
  CONSTRAINT roulette_user_pkey PRIMARY KEY (user_uuid, roulette_uuid)
);

INSERT INTO roulette (name, email, phone, description) VALUES
('Test Coffee', 'organiser@test.com', '0410111222', 'A coffee roulette for test users'),
('Dom Coffee', 'organiser2@test.com', '0441232122', 'A coffee roulette for Dom');

INSERT INTO "user" (first_name, last_name, email, phone) VALUES
('Josh', 'Test', 'test1@test.com', '0123919123'),
('Dom', 'Test', 'test2@test.com', '0411471417');

INSERT INTO "roulette_user" (user_uuid, roulette_uuid, frequency) VALUES
((SELECT uuid FROM "user" WHERE email ='test1@test.com'), (SELECT uuid FROM "roulette" WHERE email ='organiser@test.com'),3),
((SELECT uuid FROM "user" WHERE email ='test2@test.com'), (SELECT uuid FROM "roulette" WHERE email ='organiser@test.com'),3),
((SELECT uuid FROM "user" WHERE email ='test1@test.com'), (SELECT uuid FROM "roulette" WHERE email ='organiser2@test.com'),3),
((SELECT uuid FROM "user" WHERE email ='test2@test.com'), (SELECT uuid FROM "roulette" WHERE email ='organiser2@test.com'),3);
