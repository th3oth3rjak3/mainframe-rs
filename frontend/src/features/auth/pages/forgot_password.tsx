import { Button } from "@/shared/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/shared/ui/card";
import { Toaster, toast } from "sonner";

export function ForgotPassword() {
  const adminEmail = import.meta.env.VITE_ADMIN_EMAIL;

  const emailBody = "I'm unable to login to my account and need a password reset.";
  const emailSubject = "Mainframe - Password Reset Request";

  // This will be copied to the clipboard for manual pasting
  const fullEmailContent = `To: ${adminEmail}\nSubject: ${emailSubject}\n\n${emailBody}`;

  // This is for the mailto link
  const mailtoLink = `mailto:${adminEmail}?subject=${encodeURIComponent(
    emailSubject
  )}&body=${encodeURIComponent(emailBody)}`;

  // Error handling remains crucial
  if (!adminEmail) {
    return (
      <div className="flex justify-center items-center h-screen bg-background">
        <Card className="w-full max-w-md">
          <CardHeader>
            <CardTitle className="text-2xl text-destructive">Configuration Error</CardTitle>
          </CardHeader>
          <CardContent>
            <p>
              The application is not configured correctly. Please contact the site administrator.
            </p>
          </CardContent>
        </Card>
      </div>
    );
  }

  const handleCopy = () => {
    navigator.clipboard
      .writeText(fullEmailContent)
      .then(() => {
        toast.success("Full email details have been copied to your clipboard.");
      })
      .catch((err) => {
        console.error("Failed to copy text: ", err);
        toast.error("Failed to copy text to clipboard.");
      });
  };

  return (
    <>
      <Toaster richColors />
      <div className="flex justify-center items-center h-screen bg-background">
        <Card className="w-full max-w-md">
          <CardHeader>
            <CardTitle className="text-2xl">Password Reset Request</CardTitle>
            <CardDescription>
              To reset your password, please email the administrator at: <br />
              <strong className="text-foreground">{adminEmail}</strong>
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="flex flex-col space-y-4">
              <p className="text-sm text-muted-foreground">
                Click the button below to open your default email client, or copy the full details
                to paste manually.
              </p>
              <a href={mailtoLink} className="w-full">
                <Button className="w-full">Email Administrator</Button>
              </a>
              <div className="relative">
                <div className="absolute inset-0 flex items-center">
                  <span className="w-full border-t" />
                </div>
                <div className="relative flex justify-center text-xs uppercase">
                  <span className="bg-card px-2 text-muted-foreground">
                    Or copy the details below
                  </span>
                </div>
              </div>
              {/* Using a div with pre-wrap styling to respect line breaks */}
              <div
                className="rounded-md border bg-muted p-4 font-mono text-sm"
                style={{ whiteSpace: "pre-wrap" }}
              >
                {fullEmailContent}
              </div>
              <Button onClick={handleCopy} variant="outline">
                Copy Full Details
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    </>
  );
}
