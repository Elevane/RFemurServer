using System;
using System.Collections.Generic;
using System.Net.Sockets;
using System.Text;
using System.Text.Json;
using System.Threading;

class Character
{
    public int X { get; set; }
    public int Y { get; set; }
    public int Id { get; set; }
    
    public Character(int x, int y, int id)
    {
        X = x;
        Y = y;
        Id = id;
    }
}

class Client
{
    private const string Host = "127.0.0.1";
    private const int Port = 3333;
    private const int BufferSize = 1024;

    private TcpClient serverSocket;
    private NetworkStream stream;
    private string _token;
    private Character character;
    private Queue<string> messageQueue = new();
    private readonly object queueLock = new();

    public void Run()
    {
        try
        {
            serverSocket = new TcpClient(Host, Port);

            stream = serverSocket.GetStream();
            //Connection au listener TCP
            Console.WriteLine("--> Connecté au serveur TCP");
            //Connection au server de jeu
            SendMessage(" |0| ");
            
            Thread listenerThread = new(ThreadListen);
            listenerThread.IsBackground = true;
            listenerThread.Start();
        }
        catch (Exception e)
        {
            Console.WriteLine($"Impossible de se connecter : {e.Message}");
            return;
        }

        while (true)
        {
            ProcessServerMessages();
        }
    }

    private void ThreadListen()
    {
        try
        {
            byte[] buffer = new byte[BufferSize];
            while (true)
            {
                int bytesRead = stream.Read(buffer, 0, buffer.Length);
                if (bytesRead > 0)
                {
                    string data = Encoding.UTF8.GetString(buffer, 0, bytesRead);
                    lock (queueLock) { messageQueue.Enqueue(data); }
                }
            }
        }
        catch (Exception e)
        {
            Console.WriteLine($"Erreur de réception : {e.Message}");
        }
    }

    private void ProcessServerMessages()
    {
        while (messageQueue.Count > 0)
        {
            string message;
            lock (queueLock) { message = messageQueue.Dequeue(); }
            InterpretData(message);
        }
    }

    private void InterpretData(string data)
    {
        string[] parts = data.Split('|');
        if (parts.Length < 3) return;
        
        string operation = parts[1];
        
        switch (operation)
        {
            case "1": // CONNECT_SERVER_Response
                var token = parts[0];
                Console.WriteLine($"--> Token récupéré : {token}");
                character = JsonSerializer.Deserialize<Character>(parts[2]);      
                _token =  token;
                break;
            case "2": //Other player packet connect
                Console.WriteLine($"--> Other client connected {parts}");
                //character = JsonSerializer.Deserialize<Character>(parts[2]);      
                //_token =  token;
                break;
        }
    }

    private void SendMessage(string message)
    {
        byte[] data = Encoding.UTF8.GetBytes(message.PadRight(BufferSize));
        stream.Write(data, 0, data.Length);
    }
}

class Program
{
    static void Main()
    {
        Client client = new();
        client.Run();
    }
}  