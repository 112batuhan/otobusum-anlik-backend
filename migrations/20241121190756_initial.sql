CREATE TABLE bus_route_stops (
    id SERIAL PRIMARY KEY, 
    hatkodu TEXT NOT NULL,
    yon TEXT NOT NULL,
    sirano INTEGER NOT NULL,
    durakkodu TEXT NOT NULL,
    durakadi TEXT NOT NULL,
    xkoordinati DOUBLE PRECISION NOT NULL,
    ykoordinati DOUBLE PRECISION NOT NULL,
    duraktipi TEXT NOT NULL,
    isletmebolge TEXT,
    isletmealtbolge TEXT NOT NULL,
    ilceadi TEXT NOT NULL
);
