import { Button } from "@/shared/ui/button";
import { Textarea } from "@/shared/ui/textarea";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/shared/ui/card";
import { useState } from "react";
import { Copy, Check } from "lucide-react";
import type { CreateUserRequest } from "@/features/users/types";

type NewUserEmailTemplateProps = {
  newUser: CreateUserRequest;
};

const CopyIcon = ({ copied }: { copied: boolean }) =>
  copied ? <Check className="h-4 w-4" /> : <Copy className="h-4 w-4" />;

export function NewUserEmailTemplate({ newUser }: NewUserEmailTemplateProps) {
  const [templateCopied, setTemplateCopied] = useState(false);
  const [emailCopied, setEmailCopied] = useState(false);

  const emailTemplate = `Greetings ${newUser.firstName} ${newUser.lastName} and welcome to Mainframe!

We've set up your new account and you'll need to log in to change your password for security reasons.

Username: ${newUser.username}
Password: ${newUser.rawPassword}
Password Expiration: ${newUser.passwordExpiration}

Please log in at your earliest convenience to set a new password.

Best regards,
The Mainframe Team`;

  const handleCopyTemplate = async () => {
    await navigator.clipboard.writeText(emailTemplate);
    setTemplateCopied(true);
    setTimeout(() => setTemplateCopied(false), 2000);
  };

  const handleCopyEmail = async () => {
    await navigator.clipboard.writeText(newUser.email);
    setEmailCopied(true);
    setTimeout(() => setEmailCopied(false), 2000);
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Account Created Successfully</CardTitle>
        <CardDescription>
          The user account is ready. Send the login details using the template below.
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        <div className="space-y-2">
          <Label htmlFor="email-address">Email Address</Label>
          <div className="flex items-center gap-x-2">
            <Input id="email-address" type="text" readOnly value={newUser.email} />
            <Button
              type="button"
              variant="outline"
              size="sm"
              onClick={handleCopyEmail}
              className="min-w-[100px]"
            >
              <CopyIcon copied={emailCopied} />
              <span className="ml-2">{emailCopied ? "Copied!" : "Copy"}</span>
            </Button>
          </div>
        </div>
        <div className="space-y-2">
          <Label htmlFor="template">Email Template</Label>
          <Textarea
            id="template"
            value={emailTemplate}
            readOnly
            rows={12}
            className="font-mono text-sm"
          />
          <div className="flex justify-end">
            <Button
              type="button"
              variant="outline"
              size="sm"
              onClick={handleCopyTemplate}
              className="min-w-[140px]"
            >
              <CopyIcon copied={templateCopied} />
              <span className="ml-2">{templateCopied ? "Copied!" : "Copy Template"}</span>
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
