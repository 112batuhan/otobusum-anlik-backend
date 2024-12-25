-- Add up migration script here
CREATE UNIQUE INDEX timetable_route_code_city ON timetable (route_code, city);
