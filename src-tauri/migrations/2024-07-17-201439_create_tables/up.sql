-- Your SQL goes here
CREATE TABLE students (
  id INTEGER PRIMARY KEY,
  speciality_id INTEGER,
  register INTEGER,
  name TEXT,
  type TEXT,
  status TEXT,
  semester INTEGER,
  `group` TEXT,
  turn TEXT,
  level TEXT,
  FOREIGN KEY(speciality_id) REFERENCES specialities(id)
);
CREATE TABLE specialities (
  id INTEGER PRIMARY KEY,
  code INTEGER,
  name TEXT
);
CREATE TABLE subjects (
  id INTEGER PRIMARY KEY,
  teacher_id INTEGER,
  division_id INTEGER,
  code TEXT,
  name TEXT,
  FOREIGN KEY(teacher_id) REFERENCES teachers(id),
  FOREIGN KEY(division_id) REFERENCES divisions(id)
);
CREATE TABLE divisions (
  id INTEGER PRIMARY KEY,
  code INTEGER,
  academy TEXT
);
CREATE TABLE teachers (
  id INTEGER PRIMARY KEY,
  payfoll INTEGER,
  name TEXT
);
CREATE TABLE grades (
  id INTEGER PRIMARY KEY,
  student_id INTEGER,
  subject_id INTEGER,
  floults INTEGER,
  value REAL,
  weighing INTEGER,
  partial INTEGER,
  FOREIGN KEY(student_id) REFERENCES students(id),
  FOREIGN KEY(subject_id) REFERENCES subjects(id)
);
