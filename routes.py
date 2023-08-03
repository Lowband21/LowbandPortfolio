from flask import Blueprint, jsonify, send_from_directory
from models import Project, Skill, Bio

base_routes = Blueprint('base_routes', __name__)

@base_routes.route("/")
def base():
    return send_from_directory('client/public', 'index.html')

@base_routes.route("/<path:path>")
def home(path):
    return send_from_directory('client/public', path)


api = Blueprint('api', __name__)

@api.route("/api/getProjects")
def get_projects():
    projects = Project.query.all()
    return jsonify([{'id': proj.id, 'name': proj.name, 'description': proj.description} for proj in projects])

@api.route("/api/getSkills")
def get_skills():
    skills = Skill.query.all()
    return jsonify([{'id': skill.id, 'name': skill.name} for skill in skills])

@api.route("/api/getBio")
def get_bio():
    bio = Bio.query.first()  # assuming there is only one bio
    return bio.bio if bio else "Bio not found"
