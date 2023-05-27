FROM dennisjensen95/coverage-scope:v0.3.1

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
