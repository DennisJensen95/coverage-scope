FROM dennisjensen95/coverage-scope:v0.2.1

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
