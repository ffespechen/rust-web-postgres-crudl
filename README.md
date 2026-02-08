# 📚 Rust Web Postgres CRUDL

Una aplicación web profesional y robusta desarrollada con **Rust**, utilizando el framework **Axum**, el motor de plantillas **Askama** y persistencia en **PostgreSQL** con **SQLx**.

Este proyecto implementa un sistema CRUDL (Create, Read, Update, Delete, List) completo, accesible tanto a través de una **Interfaz Web** responsiva como de una **API REST**.

## 🚀 Características

- **Seguridad de Tipos**: SQLx garantiza que tus consultas SQL sean correctas en tiempo de compilación.
- **Renderizado en Servidor (SSR)**: Uso de Askama para plantillas HTML rápidas y seguras.
- **Interfaz Moderna**: Estilizada con Bootstrap 5 para una experiencia responsiva.
- **API REST**: Endpoints JSON dedicados para integración con terceros.
- **Dockerizado**: Entorno de desarrollo y producción reproducible con Docker Compose.
- **Migraciones Automáticas**: La base de datos se actualiza automáticamente al iniciar la app.

## 🛠️ Stack Tecnológico

- **Lenguaje**: Rust 🦀
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum)
- **Base de Datos**: PostgreSQL 🐘
- **SQL Toolkit**: [SQLx](https://github.com/launchbadge/sqlx)
- **Plantillas**: [Askama](https://github.com/djc/askama)
- **Infraestructura**: Docker & Docker Compose 🐳

## 📋 Requisitos Previos

- Docker y Docker Compose instalados.
- (Opcional) Rust y Cargo si deseas compilar localmente.
- (Opcional) `sqlx-cli` para gestionar migraciones manualmente.

## 📡 **Endpoints y Rutas**

La aplicación está dividida en dos grandes bloques: la interfaz visual renderizada en servidor y la interfaz de datos programática.

### 🌐 Interfaz Web (HTML)

Accesible a través de cualquier navegador. Devuelve HTML estilizado con Bootstrap.

| Funcionalidad   | Método | Ruta                    | Descripción                                                             |
| :-------------- | :----- | :---------------------- | :---------------------------------------------------------------------- |
| **Inicio**      | `GET`  | `/`                     | Bienvenida y estadísticas globales (conteo).                            |
| **Catálogo**    | `GET`  | `/web/books`            | Listado interactivo. Soporta búsqueda vía query param: `?search=valor`. |
| **Nuevo Libro** | `GET`  | `/web/books/new`        | Formulario visual para añadir un registro.                              |
| **Guardar**     | `POST` | `/web/books`            | Procesa el formulario de creación y redirige al catálogo.               |
| **Editar**      | `GET`  | `/web/books/edit/:id`   | Formulario pre-rellenado con los datos actuales del libro.              |
| **Actualizar**  | `POST` | `/web/books/update/:id` | Procesa los cambios y redirige al catálogo.                             |
| **Borrar**      | `POST` | `/web/books/delete/:id` | Elimina el registro y redirige al catálogo.                             |

### 🤖 API REST (JSON)

Diseñada para clientes externos, aplicaciones móviles o frontend desacoplado.

| Acción         | Método   | Ruta             | Cuerpo Requerido (JSON)                                    | Código Éxito     |
| :------------- | :------- | :--------------- | :--------------------------------------------------------- | :--------------- |
| **Listar**     | `GET`    | `/api/books`     | Ninguno                                                    | `200 OK`         |
| **Crear**      | `POST`   | `/api/books`     | `{"title": "...", "author": "...", "published_year": ...}` | `201 Created`    |
| **Actualizar** | `PUT`    | `/api/books/:id` | `{"title": "...", "author": "...", "published_year": ...}` | `200 OK`         |
| **Borrar**     | `DELETE` | `/api/books/:id` | Ninguno                                                    | `204 No Content` |

---

### 📝 Ejemplo de uso de la API (cURL)

**Crear un nuevo libro:**

```bash
curl -X POST http://localhost:3000/api/books \
  -H "Content-Type: application/json" \
  -d '{"title": "El Camino de los Reyes", "author": "Brandon Sanderson", "published_year": 2010}'
```

## 📁 **Estructura del Proyecto**

```text
.
├── migrations/             # Migraciones de base de datos SQL
├── src/
│   ├── handlers/           # Lógica de los endpoints
│   │   ├── mod.rs
│   │   ├── api.rs          # Lógica JSON (REST)
│   │   └── web.rs          # Lógica HTML (Askama)
│   ├── routes/             # Definición de rutas
│   │   ├── mod.rs
│   │   ├── api_routes.rs
│   │   └── web_routes.rs
│   ├── models.rs           # Modelos de datos y structs
│   └── main.rs             # Configuración y estado de la app
├── templates/              # Archivos HTML (.html)
├── .env                    # Variables de entorno
├── Dockerfile              # Build multi-stage
├── docker-compose.yml      # Orquestación de contenedores
└── entrypoint.sh           # Script de espera para la DB
```

## ⚙️ Instalación y Despliegue

1. **Clonar el repositorio:**

   ```bash
   git clone https://github.com/ffespechen/rust-web-postgres-crudl.git
   cd web-postgres-crudl

   ```

2. **Levantar el Docker Compose**

```bash
   docker-compose up --build

```
