INSERT INTO public.users (first_name, last_name, username, email, password, is_admin)
VALUES ('local', 'admin', 'admin', 'admin@localhost', '$argon2id$v=19$m=19456,t=2,p=1$y166di5g7DAGqlOqcr7oSg$GfUFCBVZ9WgYxvI+A2QNOqZ96aw4gX1Qtz5iViBmBFc', true)
ON CONFLICT (username) DO NOTHING;
