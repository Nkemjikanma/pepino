import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query"
import { checkHealth, getUsers, createUser } from "./api/users";
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

function App() {
  const queryClient = useQueryClient();


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
      queryClient.invalidateQueries({ queryKey: ['users'] })
    }
  })
  return (
    <>
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <p>
          {health}
        </p>
      </div>
      <div className="read-the-docs">
        {users?.map(user => (
          <p key={user.id}>{user.name}</p>
        ))}
      </div>
    </>
  )
}

export default App
