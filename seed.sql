-- Limpa os dados atuais
DELETE FROM assessments;
DELETE FROM classes;
DELETE FROM schools;

-- 1. Inserir 3 Escolas do Paraná
INSERT INTO schools (name, code) VALUES ('Colégio Estadual Pedro Macedo', 'PEDRO01');
INSERT INTO schools (name, code) VALUES ('Colégio Estadual de Paranaguá', 'PARAN02');
INSERT INTO schools (name, code) VALUES ('Colégio Estadual de Curitiba', 'CURIT03');

-- 2. Inserir Turmas (baseado nas disciplinas que você leciona)
-- Escolas usam os IDs 1, 2 e 3 (gerados pelo AUTOINCREMENT)
INSERT INTO classes (school_id, name) VALUES (1, 'Front-end Development - 3º Ano');
INSERT INTO classes (school_id, name) VALUES (1, 'Back-end Development - 2º Ano');
INSERT INTO classes (school_id, name) VALUES (2, 'Pensamento Computacional - 1º Ano');
INSERT INTO classes (school_id, name) VALUES (3, 'Data Science - 3º Ano');

-- 3. Inserir Avaliações "GO" (as avaliações da semana)
INSERT INTO assessments (class_id, date, subject) VALUES (1, '2026-05-18', 'Prova Prática React');
INSERT INTO assessments (class_id, date, subject) VALUES (2, '2026-05-19', 'GO - Introdução a Rust');
INSERT INTO assessments (class_id, date, subject) VALUES (3, '2026-05-20', 'GO - Lógica de Programação');
INSERT INTO assessments (class_id, date, subject) VALUES (4, '2026-05-21', 'Análise de Dados com Python');