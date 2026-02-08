#!/bin/bash
# entrypoint.sh

# Extraer el host y puerto de la variable DATABASE_URL si es necesario, 
# o usar valores por defecto.
DB_HOST="postgres_db"  # Este es el nombre del servicio en docker-compose
DB_PORT="5432"

echo "Esperando a que la base de datos en $DB_HOST:$DB_PORT esté lista..."

# Usamos 'timeout' para no esperar eternamente y 'nc' (netcat) para chequear el puerto
until timeout 2s bash -c "cat < /dev/null > /dev/tcp/$DB_HOST/$DB_PORT" 2>/dev/null; do
  echo "Postgres no responde todavía - esperando..."
  sleep 2
done

echo "¡Base de datos detectada! Iniciando la aplicación..."

# Ejecutar el binario de la aplicación
exec ./web-postgres-crudl