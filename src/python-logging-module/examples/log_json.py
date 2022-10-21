import logging
from pythonjsonlogger import jsonlogger

root_logger = logging.getLogger()
logHandler = logging.StreamHandler()
logHandler.setFormatter(jsonlogger.JsonFormatter())

root_logger.addHandler(logHandler)

root_logger.setLevel(logging.DEBUG)

logger = logging.getLogger('test')
logger.info('test')
logger.warning('test')
logger.error('test')
