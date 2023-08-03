from flask import Blueprint, jsonify, send_from_directory
from os.path import splitext
from models import Project, Skill, Bio

base_routes = Blueprint('base_routes', __name__)

@base_routes.route('/', defaults={'path': ''})
@base_routes.route('/<path:path>')
def home(path):
    # Check if path is a file by checking if it has an extension
    if splitext(path)[1]:
        return send_from_directory('client/public', path)
    else:
        return send_from_directory('client/public', 'index.html')

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