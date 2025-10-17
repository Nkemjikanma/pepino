import type { User, CreateUserRequest } from "../types/api";
const API_BASE = "/api";

export async function checkHealth() {
  const response = await fetch(`{API_BASE}/health`);

  if (!response.ok) throw new Error("Health check failed");

  return response.json();
}

export async function getUsers(): Promise<User[]> {
  const response = await fetch(`${API_BASE}/users`);

  if (!response.ok) throw new Error("Failed to fetch users");

  const data = await response.json();

  return data.response_data;
}

export async function createUser(request: CreateUserRequest): Promise<User> {
  const response = await fetch(`${API_BASE}/users`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(request),
  });

  if (!response.ok) throw new Error("Failed to create new user");

  const data = await response.json();

  return data.response_data;
}
