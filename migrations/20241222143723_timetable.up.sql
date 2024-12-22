CREATE TABLE IF NOT EXISTS timetable (
  route_code TEXT NOT NULL,
  sunday TIME[],
  monday TIME[],
  tuesday TIME[],
  wednesday TIME[],
  thursday TIME[],
  friday TIME[],
  saturday TIME[]
);
