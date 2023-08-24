from locust import HttpUser, task

class MockApiUser(HttpUser):
    @task
    def small_request(self):
        self.client.get("/api?l=114kB")