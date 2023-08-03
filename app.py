from flask import Flask, send_from_directory
import random

from models import db, create_db, fill_db, app
from routes import api, base_routes

# Register blueprints
app.register_blueprint(api)
app.register_blueprint(base_routes)

if __name__ == "__main__":
    create_db()
    fill_db()
    app.run(debug=True)
