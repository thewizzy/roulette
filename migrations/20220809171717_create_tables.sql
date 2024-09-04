CREATE TABLE IF NOT EXISTS "roulette"
(
  uuid          uuid DEFAULT gen_random_uuid(),
  admin_code    uuid DEFAULT gen_random_uuid(),
  name          VARCHAR(100),
  email         VARCHAR(100),
  description   VARCHAR,
  PRIMARY KEY (uuid)
);

CREATE TABLE IF NOT EXISTS "user"
(
  uuid      uuid DEFAULT gen_random_uuid(),
  name      VARCHAR(100),
  email     VARCHAR(100),
  phone     VARCHAR(20),
  PRIMARY KEY (uuid)
);

INSERT INTO roulette (name, email, description) VALUES
('Test Coffee', 'organiser@test.com', 'A coffee roulette for test users'),
('Dom Coffee', 'organiser2@test.comu', 'A coffee roulette for Dom');

INSERT INTO "user" (name, email, phone) VALUES
('Josh', 'test1@test.com', '0123919123'),
('Dom', 'test2@test.com', '0411471417');
