-- Actualizamos los registros antiguos que tienen el campo vacío (NULL)
-- IMPORTANTE: Asegúrate de que esta ruta coincida con la que definiste en tu código Rust
UPDATE books 
SET image_path = 'uploads/portada_generica.jpg' 
WHERE image_path IS NULL;

-- Ahora que todos tienen datos, obligamos a que siempre tenga un valor
ALTER TABLE books ALTER COLUMN image_path SET NOT NULL;