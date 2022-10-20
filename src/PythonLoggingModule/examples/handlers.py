import logging

logger = logging.getLogger(__name__)

logger.addHandler(logging.StreamHandler())
logger.addHandler(logging.FileHandler('out.log'))

logger.warning('test')
logger.error('test')
