import logging

try:
  1 / 0
except Exception as e:
  logging.error(e)


try:
  1 / 0
except Exception as e:
  logging.error('OOPS', exc_info=True)
  # OR: logging.exception('OOPS')
