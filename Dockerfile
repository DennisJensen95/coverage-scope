FROM dennisjensen95/coverage-scope:v0.4.0

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
