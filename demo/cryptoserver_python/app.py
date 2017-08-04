import random

from flask import Flask
from flask_restful import reqparse, Api, Resource


BASE62 = '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz'


def generate_secure_id(size=128):
    secure_id = []
    for _ in range(size):
        secure_id.append(BASE62[random.SystemRandom().randint(0, 61)])
    return ''.join(secure_id)


class SecureIDGenerator(Resource):
    def __init__(self):
        get_parser = reqparse.RequestParser()
        get_parser.add_argument('size', required=True, type=int, location='args')
        self.get_parser = get_parser

    def get(self):
        args = self.get_parser.parse_args()
        return generate_secure_id(args['size'])


if __name__ == '__main__':
    app = Flask(__name__)
    api = Api(app)
    api.add_resource(SecureIDGenerator, '/generate')
    app.run(port=8000)
