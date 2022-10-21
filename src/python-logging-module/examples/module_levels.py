import logging

module_logger = logging.getLogger('module_logger')
module_logger.warning('module_logger log message')

logging.warning('module_logger log message') 

logging.basicConfig(format='%(name)s %(message)s',)
module_logger = logging.getLogger('module_logger')
module_logger.warning('test')
logging.warning('test') 

