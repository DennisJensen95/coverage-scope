FROM dennisjensen95/coverage-scope:v0.1.2

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
