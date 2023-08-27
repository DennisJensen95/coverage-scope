FROM dennisjensen95/coverage-scope:v0.5.0

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
