FROM dennisjensen95/coverage-scope:v0.2.0

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
