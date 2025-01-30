object "Return" {
    code {
        {
            return(0, 0)
        }
    }

    object "Return_deployed" {
        code {
            {
                mstore(0, 42)
                return(0, 32)
            }
        }
    }
}
