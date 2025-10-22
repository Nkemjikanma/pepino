import { useState } from "react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query"
import { checkHealth, getUsers, createUser } from "./api/users";
import './App.css'

function App() {
  const queryClient = useQueryClient();
  const [email, setEmail] = useState("");
  const [name, setName] = useState("");

  // Health Check 
  const { data: health } = useQuery({
    queryKey: ['health'],
    queryFn: checkHealth,
  })


  // list users
  const { data: users, isLoading } = useQuery({
    queryKey: ['users'],
    queryFn: getUsers,
  })


  // create user mutation 
  const createMutation = useMutation({
    mutationFn: createUser,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['users'] });
      setEmail("");
      setName("");
    }
  })

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    createMutation.mutate({ email, name });
  }
  return (
    <div className="container">
      <header>
        <h1>🥒 Pepino Project</h1>
        <div className="status">
          {health?.response_data?.status === 'ok' && (
            <span className="badge success">● Connected</span>
          )}
        </div>
      </header>

      <section className="welcome">
        <h2>Welcome to your fullstack project!</h2>
        <p>Your Axum + SQLx backend and React + Vite frontend are running.</p>
      </section>

      <section className="features">
        <div className="feature-card">
          <h3>🦀 Backend</h3>
          <p>Axum + SQLx + PostgreSQL</p>
        </div>
        <div className="feature-card">
          <h3>⚡ Frontend</h3>
          <p>React + Vite + TanStack Query</p>
        </div>
        <div className="feature-card">
          <h3>🔄 Type Safe</h3>
          <p>Typeshare: Rust → TypeScript</p>
        </div>
      </section>

      <section className="demo">
        <h2>Live Demo: Users CRUD</h2>

        <div className="create-user">
          <h3>Create User</h3>
          <form onSubmit={handleSubmit}>
            <input
              type="email"
              placeholder="Email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
            <input
              type="text"
              placeholder="Name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              required
            />
            <button type="submit" disabled={createMutation.isPending}>
              {createMutation.isPending ? 'Creating...' : 'Create User'}
            </button>
          </form>
          {createMutation.isError && (
            <p className="error">Error creating user</p>
          )}
          {createMutation.isSuccess && (
            <p className="success">User created!</p>
          )}
        </div>

        <div className="user-list">
          <h3>Users</h3>
          {isLoading && <p>Loading users...</p>}
          {/* {error && <p className="error">Error loading users</p>} */}
          {users && users.length === 0 && <p>No users yet. Create one above!</p>}
          {users && users.length > 0 && (
            <ul>
              {users.map(user => (
                <li key={user.id}>
                  <strong>{user.name}</strong> - {user.email}
                </li>
              ))}
            </ul>
          )}
        </div>
      </section>

      <footer>
        <p>Edit <code>src/App.tsx</code> to get started</p>
        <p>Made with <a href="https://github.com/nkemjikanma/pepino" target="_blank">pepino</a> 🥒</p>
      </footer>
    </div>
  );
}

export default App
